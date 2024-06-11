import React, { Suspense } from "react";
import { createRoot } from "react-dom/client";
import { RecoilRoot } from "recoil";
import { Form } from "./components/form";
import { List } from "./components/list";

function Index() {
  return (
    <RecoilRoot>
      <Suspense>
        <div className="container m-auto my-12">
          <h1 className="mb-6 text-3xl text-center">Rust / TypeScript demo</h1>
          <Form></Form>
          <List></List>
        </div>
      </Suspense>
    </RecoilRoot>
  );
}

const root = document.getElementById("react");
if (root) {
  createRoot(root).render(Index());
}
