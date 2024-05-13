"use client";
import React from "react";
import { Stack, Button, Typography } from "@mui/material";
import { Theme } from "@mui/material/styles";
import { invoke } from "@tauri-apps/api/core";

import TodoCreator from "./TodoCreator";
import TodoList from "./TodoList";

const Content: React.FC = () => {
  const MTodoList = React.useMemo(() => TodoList, []);
  const [isTauri, setIsTauri] = React.useState<boolean>(false);
  const [message, setMessage] = React.useState<string>(
    `not clicked, isTauri: ${isTauri}`,
  );

  React.useEffect(() => {
    if (typeof window !== "undefined") {
      setIsTauri("isTauri" in window && !!window.isTauri);
    }
  }, []);

  const handleClick = () => {
    console.log("hoge");
    invoke<{ invokeMessage: string }>("domains::todos::controller::find")
      .then((res) => {
        setMessage(res.invokeMessage);
      })
      .catch((err) => {
        setMessage(`failed to fetch tauri: ${err}`);
      });
  };

  return (
    <Stack
      sx={{
        width: "100%",
        height: "100%",
        p: (theme: Theme) => theme.spacing(0, 3, 0, 3),
      }}
    >
      <Stack
        sx={{
          width: "100%",
          height: "100%",
          p: 3,
          alignItems: "center",
        }}
        spacing={3}
      >
        <Button variant="contained" onClick={handleClick}>
          invoke tauri
        </Button>
        <Typography>Message: {message}</Typography>
        <TodoCreator />
        <MTodoList />
      </Stack>
    </Stack>
  );
};

export default Content;
