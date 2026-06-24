import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { useEffect } from 'react';
import { ResultadoParser } from '../types';


interface Props {
  onTestCargado: (data: ResultadoParser) => void;
}

export default function UploadView({ onTestCargado }: Props) {
  const [cargando, setCargando] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragging, setDragging] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    getCurrentWebview().onDragDropEvent((event) => {
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
    }).then(fn => { unlisten = fn; });

    return () => { unlisten?.(); };
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
            <div className="upload-icon">📄</div>
            <h2>Sube tu documento de test</h2>
            <p>Arrastra un PDF o Word (.docx) aquí, o haz clic para elegir</p>
            <button className="btn-primary" onClick={e => { e.stopPropagation(); seleccionarArchivo(); }}>
              Elegir archivo
            </button>
          </>
        )}
      </div>
      {error && <div className="error-box">{error}</div>}
    </div>
  );
}
