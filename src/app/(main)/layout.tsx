import React from "react";
import type { Metadata } from "next";
import { Stack } from "@mui/material";

import Header from "./_layout/Header";

export const metadata: Metadata = {
  title: "my-todo",
  description: "sample project for Rust(web and tauri) backend",
};

type Props = {
  children: React.ReactNode;
};

const Component: React.FC<Props> = (props) => {
  const { children } = props;
  const headerHeight = "72px";
  return (
    <Stack
      sx={{
        width: "100vw",
        height: "100vh",
      }}
    >
      <Header height={headerHeight} />
      <Stack
        sx={{
          width: "100%",
          height: `calc(100vh - ${headerHeight})`,
        }}
      >
        {children}
      </Stack>
    </Stack>
  );
};

export default Component;
