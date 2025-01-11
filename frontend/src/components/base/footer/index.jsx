import { Layout } from "antd";

const { Footer } = Layout;

const AppFooter = () => {
  return (
    <Footer
      style={{
        textAlign: "center",
        padding: "0",
      }}
    >
      <p>
        © <span>{new Date().getFullYear()}</span> Syed Vilayat Ali Rizvi & Mohammad Ahmad. All rights reserved. Built with ❤️ for coders.
      </p>
    </Footer>
  );
};

export default AppFooter;
