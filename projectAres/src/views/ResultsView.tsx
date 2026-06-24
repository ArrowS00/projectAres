import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { ResultadoParser } from '../types';

interface Props {
  data: ResultadoParser;
  respuestas: Record<number, string>;
  onRepetir: () => void;
  onNuevo: () => void;
}

export default function ResultsView({ data, respuestas, onRepetir, onNuevo }: Props) {
  const total = data.preguntas.length;
  const respondidas = Object.keys(respuestas).length;
  const correctas = Object.entries(respuestas).filter(
    ([i, r]) => data.preguntas[Number(i)].correcta != null && r === data.preguntas[Number(i)].correcta
  ).length;
  const incorrectas = respondidas - correctas;
  const pct = total > 0 ? Math.round((correctas / total) * 100) : 0;

  useEffect(() => {
    invoke('guardar_resultado', {
      titulo: data.titulo,
      total,
      correctas,
      incorrectas,
    }).catch(console.error);
  }, []);

  return (
    <div className="results-view">
      <div className="results-card">
        <div className="results-pct" style={{ color: pct >= 70 ? '#1D9E75' : pct >= 50 ? '#BA7517' : '#E24B4A' }}>
          {pct}%
        </div>
        <div className="results-nota">
          {pct >= 90 ? '🏆 Excelente' : pct >= 70 ? '✅ Aprobado' : pct >= 50 ? '⚠️ Mejorable' : '❌ Suspendido'}
        </div>
        <div className="results-grid">
          <div><span className="val green">{correctas}</span><span className="lbl">Correctas</span></div>
          <div><span className="val red">{incorrectas}</span><span className="lbl">Incorrectas</span></div>
          <div><span className="val">{total - respondidas}</span><span className="lbl">Sin contestar</span></div>
        </div>
      </div>

      <div className="repaso-lista">
        <h3>Repaso de respuestas</h3>
        {data.preguntas.map((p, i) => {
          const resp = respuestas[i];
          const sinClave = p.correcta == null;
          const ok = !sinClave && resp === p.correcta;
          return (
            <div key={i} className={`repaso-item ${!resp ? '' : sinClave ? 'repaso-neutral' : ok ? 'repaso-ok' : 'repaso-mal'}`}>
              <div className="repaso-header">
                <span>{!resp ? '—' : ok ? '✓' : '✗'} P{p.num}</span>
                <span className="repaso-letras">Tu respuesta: {resp || '—'} · Correcta: {p.correcta || '?'}</span>
              </div>
              <div className="repaso-enunciado">{p.enunciado}</div>
            </div>
          );
        })}
      </div>

      <div className="results-botones">
        <button className="btn-sec" onClick={onRepetir}>↺ Repetir test</button>
        <button className="btn-primary" onClick={onNuevo}>+ Nuevo documento</button>
      </div>
    </div>
  );
}
