import { Layout } from "antd";
import Navbar from "./header";
import AppFooter from "./footer";

const { Content } = Layout;

const AppBase = ({ children }) => {
  return (
    <Layout>
      <Navbar />
      <Content style={{ minHeight: "80vh", padding: "20px" }}>
        {children}
      </Content>
      <AppFooter />
    </Layout>
  );
};

export default AppBase;
