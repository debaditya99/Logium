import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
// This is the critical link. Adjust the filename if yours is index.css or App.css
import "./App.css"; 

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);