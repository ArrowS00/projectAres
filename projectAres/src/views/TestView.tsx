import { useState, useEffect } from 'react';
import { ResultadoParser } from '../types';

interface Props {
  data: ResultadoParser;
  onFinalizar: (respuestas: Record<number, string>) => void;
}

export default function TestView({ data, onFinalizar }: Props) {
  const [actual, setActual] = useState(0);
  const [respuestas, setRespuestas] = useState<Record<number, string>>({});

  const pregunta = data.preguntas[actual];
  const respondida = respuestas[actual];
  const tieneClave = pregunta.correcta != null;
  const total = data.preguntas.length;

  const responder = (letra: string) => {
    if (respondida) return;
    setRespuestas(prev => ({ ...prev, [actual]: letra }));
  };

  const siguiente = () => {
    if (actual < total - 1) setActual(actual + 1);
  };

  const anterior = () => {
    if (actual > 0) setActual(actual - 1);
  };

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key !== 'Enter' || !respondida) return;
      if (actual < total - 1) siguiente();
      else onFinalizar(respuestas);
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [actual, respondida, respuestas, total]);

  const correctas = Object.entries(respuestas).filter(
    ([i, r]) => data.preguntas[Number(i)].correcta != null && r === data.preguntas[Number(i)].correcta
  ).length;

  return (
    <div className="test-view">
      <div className="test-header">
        <span className="test-titulo">{data.titulo}</span>
        <span className="test-contador">{actual + 1} / {total}</span>
        <span className="test-score">✓ {correctas} ✗ {Object.keys(respuestas).length - correctas}</span>
      </div>

      <div className="progress-bar-segmentada">
        {data.preguntas.map((p, i) => {
          const resp = respuestas[i];
          const correcta = p.correcta != null && resp === p.correcta;
          const incorrecta = resp && p.correcta != null && resp !== p.correcta;
          const sinClave = resp && p.correcta == null;
          let color = 'var(--border)';
          if (correcta) color = 'var(--correct)';
          else if (incorrecta) color = 'var(--incorrect)';
          else if (sinClave) color = 'var(--text-muted)';
          return (
            <div
              key={i}
              className={`progress-segmento ${i === actual ? 'activo' : ''}`}
              style={{ background: color }}
            />
          );
        })}
      </div>

      <div className="pregunta-card">
        <div className="pregunta-num">Pregunta {pregunta.num}</div>
        <div className="pregunta-texto">{pregunta.enunciado}</div>
      </div>

      <div className="opciones">
        {pregunta.opciones.map(op => {
          let clase = 'opcion';
          if (respondida) {
            if (tieneClave && op.letra === pregunta.correcta) clase += ' correcta';
            else if (tieneClave && op.letra === respondida) clase += ' incorrecta';
            else if (!tieneClave && op.letra === respondida) clase += ' seleccionada';
          }
          return (
            <button
              key={op.letra}
              className={clase}
              onClick={() => responder(op.letra)}
              disabled={!!respondida}
            >
              <span className="opcion-letra">{op.letra}</span>
              <span className="opcion-texto">{op.texto}</span>
            </button>
          );
        })}
      </div>


      <div className="nav-botones">
        <button className="btn-sec" onClick={anterior} disabled={actual === 0}>← Anterior</button>
        {actual < total - 1 && (
          <button className="btn-sec" onClick={siguiente} disabled={actual >= total - 1}>No Contestar</button>
        )}
        {actual < total - 1
          ? <button className="btn-primary" onClick={siguiente} disabled={!respondida}>Siguiente →</button>
          : <button className="btn-primary" onClick={() => onFinalizar(respuestas)} disabled={!respondida}>Ver resultados</button>
        }
      </div>
    </div>
  );
}
