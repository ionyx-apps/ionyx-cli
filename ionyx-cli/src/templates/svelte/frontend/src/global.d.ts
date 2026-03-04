declare global {
  interface Window {
    ionyx: {
      invoke: (command: string, payload?: any) => Promise<any>;
      resolveResponse: (responseId: string, response: any) => void;
    };
  }
}

export {};