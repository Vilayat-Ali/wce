import { ConfigProvider, Layout } from "antd";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Home from "./components/pages/dashboard";
import Login from "./components/pages/auth/login";
import Signup from "./components/pages/auth/signup";

function App() {
  return (
    <>
      <ConfigProvider
        theme={{
          token: {
            colorPrimary: "#673995",
            colorTextHeading: "#673995",
            colorLink: "#673995",
            colorPrimaryBorder: "#673995",
          },
        }}
      >
        <Layout>
          <Router>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/login" element={<Login />} />
              <Route path="/signup" element={<Signup />} />
            </Routes>
          </Router>
        </Layout>
      </ConfigProvider>
    </>
  );
}

export default App;
