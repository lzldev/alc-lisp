import { Editor } from "@monaco-editor/react";
import {
  add_print_callback,
  get_ast_gloo,
  print_callbacks,
  remove_print_callback,
  run,
  type Node,
  type Object,
} from "alc-lisp-wasm";
import clsx from "clsx";
import { useEffect, useRef, useState } from "react";
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
  const [messages, setMessages] = useState<string[]>([]);
  const callback = (...objects: Object[]) => {
    console.log("add_print_callback", objects);

    for (const obj of objects) {
      messages.push(JSON.stringify(obj));
    }

    setMessages([...messages]);
  };

  useEffect(() => {
    add_print_callback(callback);
    return () => {
      remove_print_callback(callback);
    };
  }, []);
  return (
    <div>
      <div>Output</div>
      <button
        className={clsx(
          "bg-violet-400 active:bg-violet-300  text-white px-4 py-1 outline-none"
        )}
        onClick={() => print_callbacks()}
      >
        print callbacks
      </button>
      <div className="flex flex-col flex-grow">
        {messages.map((message) => {
          return <div className="border-y-2">{message}</div>;
        })}
      </div>
    </div>
  );
}

export function Details() {
  const [selected, setSelected] = useState<(typeof tabs)[number]>(tabs[0]);

  const Component = components[selected];

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
      <div className="flex flex-grow">
        <Component />
      </div>
    </div>
  );
}
