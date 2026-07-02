import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ResultadoParser, ResultadoTest } from '../types';

interface Props {
  onRehacer: (data: ResultadoParser) => void;
}

export default function HistorialView({ onRehacer }: Props) {
  const [historial, setHistorial] = useState<ResultadoTest[]>([]);
  const [mezclando, setMezclando] = useState(false);

  const cargar = () => {
    invoke<ResultadoTest[]>('cargar_historial').then(setHistorial).catch(console.error);
  };

  useEffect(() => { cargar(); }, []);

  const crearTestMezclado = async () => {
    setMezclando(true);
    try {
      const data = await invoke<ResultadoParser>('crear_test_mezclado', { porTest: 10 });
      invoke('registrar_test_iniciado', { titulo: data.titulo, total: data.total, datosTest: JSON.stringify(data) }).catch(console.error);
      onRehacer(data);
    } catch (e) {
      alert(String(e));
    } finally {
      setMezclando(false);
    }
  };

  const rehacer = async (id: number) => {
    try {
      const data = await invoke<ResultadoParser>('cargar_test_desde_historial', { id });
      onRehacer(data);
    } catch (e) {
      alert('Este test no tiene datos guardados.');
    }
  };

  const borrarHistorial = async () => {
    if (!confirm('¿Borrar todo el historial?')) return;
    await invoke('limpiar_historial').catch(console.error);
    setHistorial([]);
  };

  if (historial.length === 0) {
    return <div className="historial-vacio">Aún no has realizado ningún test.</div>;
  }

  return (
    <div className="historial-view">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2>Historial de tests</h2>
        <div style={{ display: 'flex', gap: '0.5rem' }}>
          <button className="btn-primary" onClick={crearTestMezclado} disabled={mezclando || historial.length < 2} title="Crea un test nuevo con 10 preguntas de cada test del historial">
            {mezclando ? 'Creando...' : '🔀 Crear test mezclado'}
          </button>
          <button className="btn-sec" onClick={borrarHistorial}>Borrar historial</button>
        </div>
      </div>
      <div className="historial-lista">
        {historial.map(r => (
          <div key={r.id} className="historial-item" onClick={() => rehacer(r.id!)} style={{ cursor: 'pointer' }} title="Clic para volver a hacer este test">
            <div className="historial-titulo">{r.titulo}</div>
            <div className="historial-meta">{r.fecha}</div>
            {r.estado === 'iniciado' ? (
              <div className="historial-score" style={{ color: '#888' }}>Sin completar</div>
            ) : (
              <div className="historial-score" style={{ color: r.porcentaje >= 70 ? '#1D9E75' : '#E24B4A' }}>
                {Math.round(r.porcentaje)}%
              </div>
            )}
            <div className="historial-detalle">
              {r.estado === 'iniciado' ? `${r.total} preguntas` : `${r.correctas}/${r.total} correctas`}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
