import React from "react";
import { Stack, Typography, Checkbox } from "@mui/material";
import { Theme } from "@mui/material";

import { Todo } from "api/@types";
import DeleteButton from "./DeleteButton";
import CheckButton from "./CheckButton";

type Props = Todo;

const Component: React.FC<Props> = (props) => {
  const { id, text, completed } = props;
  return (
    <Stack
      sx={{
        width: "100%",
        alignItems: "center",
        justifyContent: "space-between",
        bgcolor: "white",
        color: "primary.dark",
        p: (theme: Theme) => theme.spacing(1, 3, 1, 3),
        boxShadow: "inset 0px -1px 0px rgba(255, 255, 255, 0.55)",
        "&:last-child": {
          boxShadow: "none",
        },
        "&:hover": {
          backgroundColor: "#EEFFFF",
        },
        //cursor: "pointer",
      }}
      direction="row"
      spacing={3}
    >
      <Stack spacing={1} direction="row" alignItems="center">
        <CheckButton completed={completed} id={id} />
        <Typography
          sx={{
            textDecorationLine: completed ? "line-through" : null,
          }}
        >
          {text}
        </Typography>
      </Stack>
      <DeleteButton id={id} />
    </Stack>
  );
};

export default Component;
