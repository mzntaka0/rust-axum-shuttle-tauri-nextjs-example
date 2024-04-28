"use client";
import React from "react";
import { Stack, Typography } from "@mui/material";
import { Theme } from "@mui/material/styles";

type Props = {
  height?: string;
};

const Component: React.FC<Props> = (props) => {
  const { height = "64px" } = props;
  return (
    <Stack
      sx={{
        width: "100vw",
        height,
        p: (theme: Theme) => theme.spacing(0, 3, 0, 3),
        backgroundColor: "main",
        backdropFilter: "blur(5.8px)",
        boxShadow: "0 25px 50px -12px rgba(0, 0, 0, 0.25)",
      }}
      spacing={4}
      direction="row"
      alignItems="center"
    >
      <Typography>Todo app w/ Rust</Typography>
    </Stack>
  );
};

export default Component;
