import React, { useState } from "react";
import { Layout } from "antd";
import AppHeader from "./header";
import AppSider from "./sider";
import AppDrawer from "./sider/Drawer";
import AppFooter from "./footer";

const AppBase = ({ renderTitle, children }) => {
  const [drawerVisible, setDrawerVisible] = useState(false);

  const toggleCollapse = () => {
    setDrawerVisible(!drawerVisible);
  };

  return (
    <Layout style={{ minHeight: "100vh", backgroundColor: "#001529" }}>
      <Layout.Sider
        collapsed={drawerVisible}
        style={{ backgroundColor: "#001529", height: "100vh", position: "fixed", zIndex: "10" }}
      >
        <AppSider collapsed={drawerVisible} onCollapse={toggleCollapse} />
      </Layout.Sider>
      <Layout style={{ marginLeft: drawerVisible ? 80 : 200 }}>
        <AppHeader onToggleDrawer={toggleCollapse} drawerVisible={drawerVisible} renderTitle={renderTitle} />
        <Layout.Content
          style={{
            padding: 20,
            // backgroundColor: "#001529",
            minHeight: "calc(100vh - 135px)",
            color: "#fff",
          }}
        >
          {children}
        </Layout.Content>
        <AppFooter />
      </Layout>
    </Layout>
  );
};

export default AppBase;