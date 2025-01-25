import type { AppProps } from "next/app";
import { Toaster } from 'react-hot-toast';

// redux
import {  Provider } from 'react-redux';
import reduxStore from "@/redux";

// styles
import "@/styles/globals.css";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <Provider store={reduxStore}>
      <Component {...pageProps} />
      <Toaster />
    </Provider>
  );
}
