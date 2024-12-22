import { useState, useEffect } from 'react';

export default function App() {
    const [wasmExports, setWasmExports] = useState(null);

    useEffect(() => {
        const loadWasm = async () => {
            try {
                // Dynamically import the WASM file
                const wasmModule = await import('./path-to-your-wasm-file.wasm');
                setWasmExports(wasmModule);
            } catch (error) {
                console.error('Failed to load WASM module:', error);
            }
        };

        loadWasm();
    }, []);

    if (!wasmExports) {
        return <div>Loading WASM...</div>;
    }

    return <div>WASM Loaded! You can now use it.</div>;
}
