import React from 'react'
import ReactDOM from 'react-dom/client'
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import MainScreen from "./pages/MainScreen.tsx";
import VoteScreen from "./pages/VoteScreen.tsx";
import ResultsScreen from "./pages/ResultsScreen.tsx";
import CreateScreen from "./pages/CreateScreen.tsx";
import {CssBaseline} from "@mui/material";
import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

const router = createBrowserRouter([
  {
    path: "/",
    element: <MainScreen/>
  },
  {
    path: "/crear",
    element: <CreateScreen/>
  },
  {
    path: "/:id",
    element: <VoteScreen/>
  },
  {
    path: "/:id/results",
    element: <ResultsScreen/>
  }
]);

const App = () => {
  return (
    <React.StrictMode>
      <CssBaseline />
        <RouterProvider router={router}/>
    </React.StrictMode>
  )
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <App />
)
