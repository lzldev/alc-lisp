import {
  parse_and_run,
  type Node,
  type Object,
  type Token,
} from "@alc-lisp/wasm";
import { Editor } from "@monaco-editor/react";
import clsx from "clsx";
import { useRef, useState } from "react";
import { ObjectInspector, TableInspector } from "react-inspector";
import { usePrintCallback } from "../hooks/useAlcLisp";
import { usePlaygroundStore } from "../stores/playground.store";
import { Header } from "./Header";
import { formatMessage } from "../utils/format";

import ExampleCode from "../assets/hello_world.alc?raw";

export function Home() {
  const editorRef = useRef<any>(null);

  const { setAST, setTokens, addMessages } = usePlaygroundStore();

  return (
    <div className="flex h-full w-full flex-1 flex-grow flex-col">
      <Header />
      <div className="flex h-full w-full flex-1 flex-grow flex-row items-stretch justify-stretch">
        <div className="flex w-[50%] flex-col border-r-2 border-r-violet-400 border-opacity-60">
          <div className="flex flex-row border-y-2 border-y-violet-400 border-opacity-60">
            <button
              className="bg-violet-400 px-4 py-1 text-white outline-none active:bg-violet-300"
              onClick={() => {
                const code = editorRef.current?.getValue();

                parse_and_run(
                  code,
                  (result: Object, tokens: Token[], ast: Node) => {
                    setTokens(tokens);
                    setAST(ast);
                    addMessages([JSON.stringify(result)]);
                  },
                );
              }}
            >
              Run
            </button>
          </div>
          <Editor
            onMount={(editor, monaco) => {
              editorRef.current = editor;
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
            defaultValue={ExampleCode}
          />
        </div>
        <Details />
      </div>
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
  const { tokens } = usePlaygroundStore();

  return (
    <div className={clsx("flex flex-1 flex-col", !show && "hidden")}>
      <div>Tokens</div>
      <TableInspector data={tokens} />
    </div>
  );
}

export function AST({ show }: TabProps) {
  const { AST } = usePlaygroundStore();
  return (
    <div className={clsx("flex flex-1 flex-col", !show && "hidden")}>
      <div>AST</div>
      <ObjectInspector data={AST} />
    </div>
  );
}

export function Output({ show }: TabProps) {
  const { messages, addMessages, setMessages } = usePlaygroundStore();

  usePrintCallback((...objects: Object[]) => {
    addMessages([objects.map((obj) => formatMessage(obj)).join(" ")]);
  });

  return (
    <div className={clsx("flex h-full flex-grow flex-col", !show && "hidden")}>
      <div className="flex flex-row border-b-2 border-y-violet-400 border-opacity-60">
        <button
          className={clsx(
            "flex-shrink bg-violet-400 px-4 py-1 text-white outline-none active:bg-violet-300",
          )}
          onClick={() => {
            setMessages([]);
          }}
        >
          clear
        </button>
      </div>
      <div className="flex h-full flex-col overflow-y-scroll">
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
  const [selected, setSelected] = useState<(typeof tabs)[number]>(tabs[0]);

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
