import { Editor } from "@monaco-editor/react";
import { Counter } from "../components/counter";
import { Header } from "../components/Header";
import { App } from "./App";
import { useRef } from "react";
import { run } from "alc-lisp-wasm";

export function Home() {
  const editorRef = useRef<any>(null);

  return (
    <div className="flex flex-1 h-full flex-col">
      <Header />
      <App>
        <div className="flex flex-1 flex-grow flex-row justify-stretch w-full h-full">
          <div className="flex flex-col flex-1 border-r-violet-400 border-r-2 border-opacity-60">
            <div className="flex flex-row border-y-violet-400 border-y-2 border-opacity-60">
              <button
                className="bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none"
                onClick={() => {
                  const code = editorRef.current?.getValue();

                  console.log("code", code);
                  run(code);
                }}
              >
                Run
              </button>
            </div>
            <Editor
              onMount={(editor, monaco) => {
                editorRef.current = editor;
              }}
              className="flex flex-1 h-full"
              defaultLanguage="clojure"
              theme="vs-light"
              height="100%"
              width="100%"
              options={{
                lineNumbers: "on",
                language: "lisp",
                bracketPairColorization: {
                  enabled: true,
                  independentColorPoolPerBracketType: true,
                },
              }}
              defaultValue={`; Hello World program in alc-lisp\n\n(print "Hello World")`}
            />
          </div>
          <div className="flex flex-col flex-1">
            <div className="flex flex-row border-y-violet-400 border-y-2 border-opacity-60">
              <button className="bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none">
                Output
              </button>
              <button className="bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none">
                Tokens
              </button>
              <button className="bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none">
                AST
              </button>
            </div>
            <Counter />
          </div>
        </div>
      </App>
    </div>
  );
}
