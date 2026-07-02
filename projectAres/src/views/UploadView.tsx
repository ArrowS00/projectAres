import { useState, useEffect } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { ResultadoParser } from '../types';


interface Props {
  onTestCargado: (data: ResultadoParser) => void;
}

export default function UploadView({ onTestCargado }: Props) {
  const [cargando, setCargando] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragging, setDragging] = useState(false);
  const [infoAbierta, setInfoAbierta] = useState(false);
  const [easterEgg, setEasterEgg] = useState(false);
  const [clicksGratuito, setClicksGratuito] = useState(0);

  useEffect(() => {
    const unlistenPromise = getCurrentWebview().onDragDropEvent((event) => {
      if (event.payload.type === 'over') {
        setDragging(true);
      } else if (event.payload.type === 'leave') {
        setDragging(false);
      } else if (event.payload.type === 'drop') {
        setDragging(false);
        const paths = event.payload.paths;
        if (paths && paths.length > 0) {
          procesarRuta(paths[0]);
        }
      }
    });

    return () => { unlistenPromise.then(fn => fn()); };
  }, []);

  const procesarRuta = async (ruta: string) => {
    setError(null);
    const ext = ruta.toLowerCase().split('.').pop();
    if (!['pdf', 'docx', 'odt', 'odf'].includes(ext ?? '')) {
      setError('Formato no soportado. Usa PDF, .docx o .odt');
      return;
    }
    setCargando(true);
    try {
      const resultado = await invoke<ResultadoParser>('procesar_archivo', { ruta });
      if (resultado.total === 0) {
        setError('No se encontraron preguntas en el documento.');
        return;
      }
      invoke('registrar_test_iniciado', { titulo: resultado.titulo, total: resultado.total, datosTest: JSON.stringify(resultado) }).catch(console.error);
      onTestCargado(resultado);
    } catch (e) {
      setError(String(e));
    } finally {
      setCargando(false);
    }
  };

  const seleccionarArchivo = async () => {
    setError(null);
    const ruta = await open({
      filters: [{ name: 'Documentos', extensions: ['pdf', 'docx', 'odt', 'odf'] }],
      multiple: false,
    });
    if (!ruta || typeof ruta !== 'string') return;
    procesarRuta(ruta);
  };

  return (
    <div className="upload-view">
      <div
        className={`upload-zone ${dragging ? 'dragging' : ''}`}
        onClick={seleccionarArchivo}
      >
        {cargando ? (
          <div className="spinner-wrap">
            <div className="spinner" />
            <p>Leyendo documento...</p>
          </div>
        ) : (
          <>
            <div className="upload-icon"><img src="/logo.png" alt="logo" style={{ width: 64, height: 64 }} /></div>
            <h2>Sube tu documento de test</h2>
            <p>Arrastra un PDF o Word (.docx) aquí, o haz clic para elegir</p>
            <button className="btn-primary" onClick={e => { e.stopPropagation(); seleccionarArchivo(); }}>
              Elegir archivo
            </button>
          </>
        )}
      </div>
      <button className="formato-info-btn" onClick={() => setInfoAbierta(true)}>ℹ️ Acerca de ProjectAres</button>

      {infoAbierta && (
        <div className="modal-overlay" onClick={() => setInfoAbierta(false)}>
          <div className="modal" onClick={e => e.stopPropagation()}>
            <h3>Acerca de ProjectAres</h3>
            <p>Hola, soy Carlos, el creador de ProjectAres, este es un proyecto <strong style={{ cursor: 'pointer' }} onClick={() => {
              const n = clicksGratuito + 1;
              setClicksGratuito(n);
              if (n >= 5) { setEasterEgg(true); setClicksGratuito(0); setInfoAbierta(false); }
            }}>GRATUITO</strong> creado para ayudarte a estudiar mediante tests, puedes usarlo y compartirlo libremente, siempre que no sea con fines lucrativos.</p>
            <hr />
            <h4>Formato del documento</h4>
            <p>Las preguntas deben estar numeradas y las opciones con letra:</p>
            <pre>{`1. ¿Pregunta?
a) Opción A
b) Opción B
c) Opción C
d) Opción D`}</pre>
            <p>Las respuestas correctas pueden ir al final del documento bajo el título <strong>SOLUCIONES</strong>:</p>
            <pre>{`SOLUCIONES
1. B
2. C`}</pre>
            <button className="btn-primary" onClick={() => setInfoAbierta(false)}>Cerrar</button>
          </div>
        </div>
      )}

      {easterEgg && (
        <div className="modal-overlay" onClick={() => setEasterEgg(false)}>
          <div className="easter-egg-card" onClick={e => e.stopPropagation()}>
            <div className="easter-egg-corazon">♥</div>
            <p className="easter-egg-texto">violeta y ares</p>
          </div>
        </div>
      )}

      {error && <div className="error-box">{error}</div>}
    </div>
  );
}
