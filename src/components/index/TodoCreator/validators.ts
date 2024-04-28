import { z } from "zod";

import { requiredErr, stringMinErr } from "lib/errorMessages";

export const Schema = z.object({
  text: z
    .string({
      required_error: requiredErr(),
    })
    .min(1, { message: stringMinErr(1) }),
});

export type FormType = z.infer<typeof Schema>;
