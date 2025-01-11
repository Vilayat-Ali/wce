
import React from "react";
import { Drawer } from "antd";
import AppSider from "./index";

const AppDrawer = ({ closeDrawer, drawerVisible }) => {
  return (
    <Drawer
      title="Menu"
      placement="left"
      closable
      onClose={closeDrawer}
      open={drawerVisible}
      bodyStyle={{ padding: 0, backgroundColor: "#001529" }}
    >
      <AppSider collapsed={drawerVisible} onCollapse={closeDrawer} />
    </Drawer>
  );
};

export default AppDrawer;