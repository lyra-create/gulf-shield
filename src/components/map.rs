use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::data::defenses::defense_systems;
use crate::data::missiles::missiles;
use crate::data::sites::{launch_sites, target_sites};

/// Build the JavaScript string that initialises Leaflet, draws trajectories,
/// interception debris ellipses, defense systems, and site markers.
fn build_map_js() -> String {
    let launches = launch_sites();
    let targets = target_sites();
    let msls = missiles();
    let defenses = defense_systems();

    let mut js = String::from(
        r#"
(function() {
    if (window._gulfShieldMap) { return; }

    var map = L.map('map', { zoomControl: true }).setView([24.5, 54.5], 6);
    window._gulfShieldMap = map;

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; OpenStreetMap contributors',
        maxZoom: 18
    }).addTo(map);

    var launchIcon = L.divIcon({
        className: 'launch-marker',
        html: '<div style="width:10px;height:10px;border-radius:50%;background:#7f8c8d;border:2px solid #2c3e50;"></div>',
        iconSize: [14, 14],
        iconAnchor: [7, 7]
    });

    var targetIcon = L.divIcon({
        className: 'target-marker',
        html: '<div style="width:12px;height:12px;border-radius:50%;background:#3498db;border:2px solid #2980b9;"></div>',
        iconSize: [16, 16],
        iconAnchor: [8, 8]
    });

    var defenseIcon = L.divIcon({
        className: 'defense-marker',
        html: '<div style="width:14px;height:14px;border-radius:3px;background:#27ae60;border:2px solid #1e8449;display:flex;align-items:center;justify-content:center;color:white;font-size:9px;font-weight:bold;">D</div>',
        iconSize: [18, 18],
        iconAnchor: [9, 9]
    });

    // Helper: create an ellipse as a polygon (Leaflet doesn't have native ellipses)
    function createDebrisEllipse(centerLat, centerLng, semiMajorM, semiMinorM, rotationDeg, options) {
        var points = [];
        var earthR = 6371000;
        var rotRad = rotationDeg * Math.PI / 180;
        for (var i = 0; i <= 36; i++) {
            var angle = (i / 36) * 2 * Math.PI;
            // Point on unrotated ellipse
            var x = semiMajorM * Math.cos(angle);
            var y = semiMinorM * Math.sin(angle);
            // Rotate
            var xr = x * Math.cos(rotRad) - y * Math.sin(rotRad);
            var yr = x * Math.sin(rotRad) + y * Math.cos(rotRad);
            // Convert to lat/lng offset
            var dlat = yr / earthR * (180 / Math.PI);
            var dlng = xr / (earthR * Math.cos(centerLat * Math.PI / 180)) * (180 / Math.PI);
            points.push([centerLat + dlat, centerLng + dlng]);
        }
        return L.polygon(points, options);
    }
"#,
    );

    // --- Defense system markers + range circles ---
    for d in &defenses {
        js.push_str(&format!(
            "L.marker([{lat},{lng}], {{icon: defenseIcon}}).addTo(map)\
             .bindTooltip('{name} ({dtype})', {{permanent:false, direction:'top'}});\n",
            lat = d.lat,
            lng = d.lng,
            name = d.name,
            dtype = d.type_label()
        ));
        // Show defense engagement range as a subtle green circle
        js.push_str(&format!(
            "L.circle([{lat},{lng}], {{radius:{r}, color:'rgba(39,174,96,0.3)', fillColor:'rgba(39,174,96,0.05)', fillOpacity:0.05, weight:1, dashArray:'4 4'}}).addTo(map)\
             .bindTooltip('Defense range: {rk}km', {{permanent:false, direction:'center'}});\n",
            lat = d.lat, lng = d.lng, r = d.range_km * 1000, rk = d.range_km
        ));
    }

    // --- Launch-site markers ---
    for s in &launches {
        js.push_str(&format!(
            "L.marker([{lat},{lng}], {{icon: launchIcon}}).addTo(map).bindTooltip('{name}', {{permanent:false, direction:'top'}});\n",
            lat = s.lat, lng = s.lng, name = s.name
        ));
    }

    // --- Target markers ---
    for t in &targets {
        js.push_str(&format!(
            "L.marker([{lat},{lng}], {{icon: targetIcon}}).addTo(map).bindTooltip('{name}', {{permanent:false, direction:'top'}});\n",
            lat = t.lat, lng = t.lng, name = t.name
        ));
    }

    // --- Trajectory arcs + interception debris ellipses ---
    for launch in &launches {
        for target in &targets {
            let dx = target.lng - launch.lng;
            let dy = target.lat - launch.lat;
            let dist_deg = (dx * dx + dy * dy).sqrt();
            let dist_km = dist_deg * 111.0;

            let missile = msls
                .iter()
                .filter(|m| m.range_km as f64 >= dist_km * 0.5)
                .min_by_key(|m| ((m.range_km as f64) - dist_km).abs() as u64)
                .unwrap_or(&msls[0]);

            // Build a quadratic Bézier arc
            let mid_lat = (launch.lat + target.lat) / 2.0 + dist_deg * 0.18;
            let mid_lng = (launch.lng + target.lng) / 2.0;

            let mut points = String::from("[");
            for i in 0..=20 {
                let t_f = i as f64 / 20.0;
                let inv = 1.0 - t_f;
                let lat =
                    inv * inv * launch.lat + 2.0 * inv * t_f * mid_lat + t_f * t_f * target.lat;
                let lng =
                    inv * inv * launch.lng + 2.0 * inv * t_f * mid_lng + t_f * t_f * target.lng;
                if i > 0 {
                    points.push(',');
                }
                points.push_str(&format!("[{:.4},{:.4}]", lat, lng));
            }
            points.push(']');

            // --- Calculate interception point ---
            // Interception happens in terminal phase, roughly 20-40km before target
            // (defense systems engage incoming missiles as they approach)
            // Place the interception point ~85% along the trajectory toward target
            let intercept_t = 0.85;
            let inv_i = 1.0 - intercept_t;
            let intercept_lat = inv_i * inv_i * launch.lat
                + 2.0 * inv_i * intercept_t * mid_lat
                + intercept_t * intercept_t * target.lat;
            let intercept_lng = inv_i * inv_i * launch.lng
                + 2.0 * inv_i * intercept_t * mid_lng
                + intercept_t * intercept_t * target.lng;

            // Rotation angle of the debris ellipse = trajectory bearing at intercept point
            let bearing_deg = (dx).atan2(dy) * 180.0 / std::f64::consts::PI;

            let debris_len = missile.intercept_debris_length_m();
            let debris_wid = missile.intercept_debris_width_m();
            let intercept_alt = missile.intercept_altitude_km();

            // Draw debris ellipse at interception point
            js.push_str(&format!(
                "createDebrisEllipse({lat}, {lng}, {major}, {minor}, {rot}, \
                 {{color:'rgba(230,160,40,0.4)', fillColor:'rgba(230,160,40,0.12)', \
                 fillOpacity:0.12, weight:1.5, dashArray:'3 3'}}).addTo(map)\
                 .bindPopup('<div class=\"debris-popup\">\
                 <h4>Interception Debris Zone</h4>\
                 <div class=\"detail\"><b>Trajectory:</b> {from} → {to}</div>\
                 <div class=\"detail\"><b>Missile:</b> {mname}</div>\
                 <div class=\"detail\"><b>Intercept altitude:</b> ~{alt}km</div>\
                 <div class=\"detail\"><b>Debris field:</b> {dlen}m × {dwid}m</div>\
                 <div class=\"detail warn\">⚠ Falling debris from intercepted missiles can still cause harm. Stay indoors during alerts.</div>\
                 </div>');\n",
                lat = intercept_lat,
                lng = intercept_lng,
                major = debris_len,
                minor = debris_wid,
                rot = bearing_deg,
                from = launch.name,
                to = target.name,
                mname = missile.name,
                alt = intercept_alt,
                dlen = debris_len,
                dwid = debris_wid,
            ));

            // Draw the trajectory arc
            let route_label = format!("{} → {}", launch.name, target.name);
            let popup_html = format!(
                "<div class=\\'trajectory-popup\\'>\
                 <h4>{}</h4>\
                 <div class=\\'detail\\'><b>Missile type:</b> {}</div>\
                 <div class=\\'detail\\'><b>Range:</b> {} km</div>\
                 <div class=\\'detail\\'><b>Est. flight time:</b> {} min</div>\
                 <div class=\\'detail\\'><b>Intercept altitude:</b> ~{} km</div>\
                 <div class=\\'detail\\'><b>Debris scatter:</b> {}m × {}m ellipse</div>\
                 <div class=\\'detail note\\'>Debris falls along the incoming trajectory after interception. \
                 The ellipse shows the approximate area where fragments may land.</div>\
                 </div>",
                route_label,
                missile.name,
                missile.range_km,
                missile.flight_time_min,
                intercept_alt,
                debris_len,
                debris_wid,
            );

            js.push_str(&format!(
                "L.polyline({points}, {{color:'#7f8c8d', weight:1.5, dashArray:'6 4', opacity:0.6}}).addTo(map)\
                 .on('mouseover', function(e){{ e.target.setStyle({{color:'#2980b9', weight:2.5, opacity:1}}); }})\
                 .on('mouseout',  function(e){{ e.target.setStyle({{color:'#7f8c8d', weight:1.5, opacity:0.6}}); }})\
                 .bindPopup('{popup}');\n",
                points = points,
                popup = popup_html
            ));
        }
    }

    // --- Legend ---
    js.push_str(r#"
    var legend = L.control({position: 'bottomleft'});
    legend.onAdd = function(map) {
        var div = L.DomUtil.create('div', 'map-legend');
        div.innerHTML = '<div class="legend-title">Legend</div>' +
            '<div class="legend-item"><span class="legend-dot" style="background:#7f8c8d;border-color:#2c3e50;"></span> Launch site</div>' +
            '<div class="legend-item"><span class="legend-dot" style="background:#3498db;border-color:#2980b9;"></span> Target area</div>' +
            '<div class="legend-item"><span class="legend-dot" style="background:#27ae60;border-color:#1e8449;border-radius:3px;"></span> Defense system</div>' +
            '<div class="legend-item"><span class="legend-ellipse"></span> Intercept debris zone</div>' +
            '<div class="legend-item"><span class="legend-line"></span> Potential trajectory</div>';
        return div;
    };
    legend.addTo(map);
"#);

    js.push_str("})();\n");
    js
}

/// The Leaflet map component.
#[component]
pub fn MapView() -> impl IntoView {
    let map_js = build_map_js();

    Effect::new(move |_| {
        let js = map_js.clone();
        let cb = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            let _ = js_sys::eval(&js);
        }) as Box<dyn Fn()>);
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                cb.as_ref().unchecked_ref(),
                100,
            );
        cb.forget();
    });

    view! {
        <div class="map-container">
            <div id="map"></div>
        </div>
    }
}
