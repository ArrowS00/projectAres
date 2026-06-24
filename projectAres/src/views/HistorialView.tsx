import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ResultadoTest } from '../types';

export default function HistorialView() {
  const [historial, setHistorial] = useState<ResultadoTest[]>([]);

  useEffect(() => {
    invoke<ResultadoTest[]>('cargar_historial').then(setHistorial).catch(console.error);
  }, []);

  if (historial.length === 0) {
    return <div className="historial-vacio">Aún no has realizado ningún test.</div>;
  }

  return (
    <div className="historial-view">
      <h2>Historial de tests</h2>
      <div className="historial-lista">
        {historial.map(r => (
          <div key={r.id} className="historial-item">
            <div className="historial-titulo">{r.titulo}</div>
            <div className="historial-meta">{r.fecha}</div>
            <div className="historial-score" style={{ color: r.porcentaje >= 70 ? '#1D9E75' : '#E24B4A' }}>
              {Math.round(r.porcentaje)}%
            </div>
            <div className="historial-detalle">{r.correctas}/{r.total} correctas</div>
          </div>
        ))}
      </div>
    </div>
  );
}
