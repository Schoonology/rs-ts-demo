import React, { Suspense } from "react";
import { createRoot } from "react-dom/client";
import { RecoilRoot } from "recoil";
import { Form } from "./components/form";
import { List } from "./components/list";

function Index() {
  return (
    <RecoilRoot>
      <Suspense>
        <Form></Form>
        <List></List>
      </Suspense>
    </RecoilRoot>
  );
}

const root = document.getElementById("react");
if (root) {
  createRoot(root).render(Index());
}
