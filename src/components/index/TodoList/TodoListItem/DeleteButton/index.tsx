import React from "react";
import { IconButton } from "@mui/material";
import { DeleteOutlined } from "@mui/icons-material";
import { useQueryClient, useMutation } from "react-query";
import aspida from "@aspida/axios";
import { useSnackbar } from "notistack";

import { api as axiosApi } from "lib/fetcher";
import api from "api/$api";
import useDialog from "lib/hooks/useDialog";
import { Todo } from "api/@types";
import { deleteTodo } from "components/index/common/functions/deleteTodo";
import DialogContent from "./DialogContent";

type Props = Pick<Todo, "id">;

const Component: React.FC<Props> = (props) => {
  const { id } = props;
  const { open, close, Dialog } = useDialog();

  const { enqueueSnackbar } = useSnackbar();

  const client = api(aspida(axiosApi));
  const queryClient = useQueryClient();
  const { mutateAsync } = useMutation(deleteTodo);

  const handleOk = async () => {
    await mutateAsync(
      { id },
      {
        onSuccess: async () => {
          await queryClient.invalidateQueries(client.todos.$path());
          enqueueSnackbar("Todo has been deleted", {
            variant: "success",
          });
        },
        onError: (_err: any) => {
          enqueueSnackbar("Failed to delete a todo", {
            variant: "error",
          });
        },
      },
    );
    close();
  };
  return (
    <>
      <IconButton>
        <DeleteOutlined
          sx={{
            color: "error.main",
          }}
          onClick={open}
        />
      </IconButton>
      <Dialog title="delete todo" onOk={handleOk} footer>
        <DialogContent />
      </Dialog>
    </>
  );
};

export default Component;
