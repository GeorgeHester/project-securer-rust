import React from "react";
import ReactDOM from "react-dom/client";
import { LoginForm } from "./components/login";

const rootElement = document.getElementById("root");

if (rootElement === null) throw new Error("Root element not found in index.html");

const reactRoot = ReactDOM.createRoot(rootElement);

reactRoot.render(
    <React.StrictMode>
        <LoginForm>
        </LoginForm>
    </React.StrictMode>,
);