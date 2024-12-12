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
    <div className="flex h-full w-full flex-1 flex-grow flex-col">
      <Header />
      <App>
        <div className="flex h-full w-full flex-1 flex-grow flex-row items-stretch justify-stretch">
          <div className="flex w-[50%] flex-col border-r-2 border-r-violet-400 border-opacity-60">
            <div className="flex flex-row border-y-2 border-y-violet-400 border-opacity-60">
              <button
                className="bg-violet-400 px-4 py-1 text-white outline-none active:bg-violet-300"
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
              className="flex h-full flex-1"
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
          <Details />
        </div>
      </App>
    </div>
  );
}

const tabs = ["Output", "Tokens", "AST"] as const;

type TabProps = {
  show?: boolean;
};

const components: Record<(typeof tabs)[number], React.FC<TabProps & any>> = {
  AST,
  Tokens,
  Output,
};

export function Tokens({ show }: TabProps) {
  return (
    <div className={clsx("flex flex-1 flex-col", !show && "hidden")}>
      Tokens
    </div>
  );
}

export function AST({ show }: TabProps) {
  return (
    <div className={clsx("flex flex-1 flex-col", !show && "hidden")}>AST</div>
  );
}

export function Output({ show }: TabProps) {
  const [messages, setMessages] = useState<string[]>(["Hello World"]);
  const callback = (...objects: Object[]) => {
    console.log("add_print_callback", objects);

    for (const obj of objects) {
      messages.push(JSON.stringify(obj));
    }

    setMessages([...messages]);
  };

  useEffect(() => {
    try {
      add_print_callback(callback);
      return () => {
        remove_print_callback(callback);
      };
    } catch (err) {
      console.error(err);
    }
  }, []);

  return (
    <div className={clsx("flex h-full flex-grow flex-col", !show && "hidden")}>
      <button
        className={clsx(
          "flex-shrink bg-violet-400 px-4 py-1 text-white outline-none active:bg-violet-300",
        )}
        onClick={() => print_callbacks()}
      >
        print callbacks
      </button>
      <div className="flex flex-col overflow-y-scroll">
        {messages.map((message, idx) => {
          return (
            <div key={idx} className="border-b-2">
              {message}
            </div>
          );
        })}
      </div>
    </div>
  );
}

export function Details() {
  const [selected, setSelected] = useState<(typeof tabs)[number]>(tabs[1]);

  return (
    <div className="flex flex-1 flex-col">
      <div className="flex flex-row border-y-2 border-y-violet-400 border-opacity-60">
        {tabs.map((tab) => (
          <button
            key={tab}
            className={clsx(
              "bg-violet-400 px-4 py-1 text-white outline-none active:bg-violet-300",
              tab === selected && "font-bold",
            )}
            onClick={() => setSelected(tab)}
          >
            {tab}
          </button>
        ))}
      </div>
      {tabs.map((tab) => {
        const Component = components[tab];
        return <Component key={tab} show={tab === selected} />;
      })}
    </div>
  );
}
