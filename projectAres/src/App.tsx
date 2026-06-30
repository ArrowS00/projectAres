import { useState } from 'react';
import { ResultadoParser, Vista } from './types';
import UploadView from './views/UploadView';
import TestView from './views/TestView';
import ResultsView from './views/ResultsView';
import HistorialView from './views/HistorialView';
import './App.css';

export default function App() {
  const [vista, setVista] = useState<Vista>('upload');
  const [testData, setTestData] = useState<ResultadoParser | null>(null);
  const [respuestas, setRespuestas] = useState<Record<number, string>>({});

  const iniciarTest = (data: ResultadoParser) => {
    setTestData(data);
    setRespuestas({});
    setVista('test');
  };

  const finalizarTest = (resp: Record<number, string>) => {
    setRespuestas(resp);
    setVista('resultados');
  };

  return (
    <div className="app">
      <nav className="navbar">
        <span className="navbar-title">ProjectAres</span>
        <div className="navbar-links">
          <button onClick={() => setVista('upload')} className={vista === 'upload' ? 'active' : ''}>Nuevo test</button>
          <button onClick={() => setVista('historial')} className={vista === 'historial' ? 'active' : ''}>Historial</button>
        </div>
      </nav>

      <main className="main">
        {vista === 'upload' && <UploadView onTestCargado={iniciarTest} />}
        {vista === 'test' && testData && (
          <TestView data={testData} onFinalizar={finalizarTest} />
        )}
        {vista === 'resultados' && testData && (
          <ResultsView
            data={testData}
            respuestas={respuestas}
            onRepetir={() => { setRespuestas({}); setVista('test'); }}
            onNuevo={() => setVista('upload')}
          />
        )}
        {vista === 'historial' && <HistorialView onRehacer={iniciarTest} />}
      </main>
    </div>
  );
}
