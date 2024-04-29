import React from "react";
import { Stack, Typography, Checkbox } from "@mui/material";
import { useQueryClient, useMutation } from "react-query";
import aspida from "@aspida/axios";
import { useSnackbar } from "notistack";

import { api as axiosApi } from "lib/fetcher";
import api from "api/$api";
import { Todo } from "api/@types";
import { updateTodo } from "components/index/common/functions/updateTodo";

type Props = Pick<Todo, "id" | "completed">;

const Component: React.FC<Props> = (props) => {
  const { id, completed } = props;

  const { enqueueSnackbar } = useSnackbar();

  const client = api(aspida(axiosApi));
  const queryClient = useQueryClient();
  const { mutateAsync } = useMutation(updateTodo);

  const handleCheck = async () => {
    await mutateAsync(
      { completed: !completed, id },
      {
        onSuccess: async () => {
          await queryClient.invalidateQueries(client.todos.$path());
          enqueueSnackbar("Todo has been updated", {
            variant: "success",
          });
        },
        onError: (_err: any) => {
          enqueueSnackbar("Failed to update a todo", {
            variant: "error",
          });
        },
      },
    );
  };
  return (
    <Stack>
      <Checkbox checked={completed} onClick={handleCheck} />
    </Stack>
  );
};

export default Component;
