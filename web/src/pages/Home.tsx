import { Editor } from "@monaco-editor/react";
import { get_ast_gloo, run, type Node } from "alc-lisp-wasm";
import clsx from "clsx";
import { useRef, useState } from "react";
import { Header } from "../components/Header";
import { App } from "./App";

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

                  const gloo_message = "gloo";
                  console.time(gloo_message);
                  get_ast_gloo(code, (node: Node) => {
                    console.timeEnd(gloo_message);
                    console.log("[gloo] node:", node);
                  });

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
              onChange={(v) => {
                console.log("change", v);
              }}
              onValidate={() => {
                console.log("validate");
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
            <Details />
          </div>
        </div>
      </App>
    </div>
  );
}

const tabs = ["Output", "Tokens", "AST"] as const;

const components: Record<(typeof tabs)[number], React.FC> = {
  AST,
  Tokens,
  Output,
};

export function Tokens() {
  return <div>Tokens</div>;
}

export function AST() {
  return <div>AST</div>;
}

export function Output() {
  return <div>Output</div>;
}

export function Details() {
  const [selected, setSelected] = useState<(typeof tabs)[number]>(tabs[0]);

  return (
    <div className="flex flex-col flex-1">
      <div className="flex flex-row border-y-violet-400 border-y-2 border-opacity-60">
        {tabs.map((tab) => (
          <button
            key={tab}
            className={clsx(
              "bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none",
              tab === selected && "font-bold"
            )}
            onClick={() => setSelected(tab)}
          >
            {tab}
          </button>
        ))}
      </div>
      <div className="flex flex-grow">{components[selected]({})}</div>
    </div>
  );
}
