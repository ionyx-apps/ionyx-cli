declare global {
  interface Window {
    ionyx: {
      invoke: (channel: string, data?: any) => Promise<any>;
      on: (channel: string, callback: (event: any, data: any) => void) => void;
      off: (channel: string, callback: (event: any, data: any) => void) => void;
    };
  }
}

export {};
