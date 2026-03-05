/* Leaflet sentiment map: init from GeoJSON, color regions by alignment (0–1). */
window.PolitechMap = (function () {
  var map, geoLayer;
  var alignmentByKey = {};
  var alignmentMin = 0;
  var alignmentSpan = 1;

  /* No topic selected: neutral gray; with alignment (0–1): green scale light → dark. */
  function colorFor(alignment) {
    if (alignment == null) return '#e2e8f0';
    var t = Math.max(0, Math.min(1, alignment));
    /* Light green #ecfdf5 (236,253,245) → dark green #166534 (22,101,52) */
    var r = Math.round(236 - t * 214);
    var g = Math.round(253 - t * 152);
    var b = Math.round(245 - t * 193);
    return 'rgb(' + r + ',' + g + ',' + b + ')';
  }

  /* 0 or null → gray; otherwise normalize over data range so green scale is amplified. */
  function colorValueFor(raw) {
    if (raw == null || raw === 0) return null;
    if (alignmentSpan <= 0) return raw;
    return (raw - alignmentMin) / alignmentSpan;
  }

  function styleFor(feature) {
    var key = feature.properties && feature.properties.adm1_en;
    var raw = key ? alignmentByKey[key] : null;
    var t = colorValueFor(raw);
    return {
      fillColor: t != null ? colorFor(t) : colorFor(null),
      weight: 1,
      opacity: 1,
      color: raw != null ? '#15803d' : '#94a3b8',
      fillOpacity: 0.78
    };
  }

  return {
    init: function (containerId, geojsonUrl) {
      if (map) return;
      var L = window.L;
      if (!L) {
        console.error('Leaflet not loaded');
        return;
      }
      map = L.map(containerId, {
        zoomControl: false,
        attributionControl: false
      }).setView([12.5, 122], 7);
      L.control.zoom({ position: 'topright' }).addTo(map);

      fetch(geojsonUrl)
        .then(function (r) { return r.json(); })
        .then(function (data) {
          geoLayer = L.geoJSON(data, {
            style: styleFor,
            onEachFeature: function (feature, layer) {
              var key = feature.properties && feature.properties.adm1_en;
              var raw = key ? alignmentByKey[key] : null;
              var msg = key || 'Unknown';
              if (raw != null) msg += ' — ' + Math.round(raw * 100) + '%';
              layer.bindTooltip(msg);
            }
          }).addTo(map);
          map.fitBounds(geoLayer.getBounds(), { padding: [16, 16], maxZoom: 8 });
          setTimeout(function () { map.invalidateSize(); }, 0);
        })
        .catch(function (e) { console.error('GeoJSON load failed', e); });
    },
    setAlignment: function (alignmentArray) {
      alignmentByKey = {};
      alignmentMin = 0;
      alignmentSpan = 1;
      if (Array.isArray(alignmentArray) && alignmentArray.length > 0) {
        var vals = alignmentArray.map(function (r) { return r.alignment_01; }).filter(function (v) { return typeof v === 'number'; });
        if (vals.length > 0) {
          alignmentMin = Math.min.apply(null, vals);
          var max = Math.max.apply(null, vals);
          alignmentSpan = Math.max(max - alignmentMin, 1e-9);
        }
        alignmentArray.forEach(function (r) {
          if (r.geojson_key) alignmentByKey[r.geojson_key] = r.alignment_01;
        });
      }
      if (geoLayer) {
        geoLayer.eachLayer(function (layer) {
          layer.setStyle(styleFor(layer.feature));
        });
      }
    },
    invalidateSize: function () {
      if (map) setTimeout(function () { map.invalidateSize(); }, 50);
    }
  };
})();
