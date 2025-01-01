import { Layout } from "antd";

const { Footer } = Layout;

const AppFooter = () => {
  return (
    <Footer
      style={{
        textAlign: "center",
        backgroundColor: "#f0f0f0",
        padding: "0",
      }}
    >
      <p>
        © 2025 Syed Vilayat Ali Rizvi & Mohammad Ahmad. All rights reserved. Built with ❤️ for coders.
      </p>
    </Footer>
  );
};

export default AppFooter;
