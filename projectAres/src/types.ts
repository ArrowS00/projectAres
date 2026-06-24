export interface Opcion {
  letra: string;
  texto: string;
}

export interface Pregunta {
  num: number;
  enunciado: string;
  opciones: Opcion[];
  correcta: string | null;
}

export interface ResultadoParser {
  titulo: string;
  preguntas: Pregunta[];
  total: number;
  con_clave: boolean;
}

export interface ResultadoTest {
  id?: number;
  titulo: string;
  fecha: string;
  total: number;
  correctas: number;
  incorrectas: number;
  porcentaje: number;
}

export type Vista = 'upload' | 'test' | 'resultados' | 'historial';
