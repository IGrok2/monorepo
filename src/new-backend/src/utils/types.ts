import { type Routes } from "../wrpc/theoretical";
import { Login, LoginInput, LoginOutput } from "../routes/auth/login";
import {
  Register,
  RegisterInput,
  RegisterOutput,
} from "../routes/auth/register";
import {
  VerifyEmail,
  VerifyEmailInput,
  VerifyEmailOutput,
} from "../routes/auth/verify";
import {
  ResetPasswordSend,
  ResetPasswordSendInput,
  ResetPasswordSendOutput,
  ResetPasswordComplete,
  ResetPasswordCompleteInput,
  ResetPasswordCompleteOutput,
} from "../routes/auth/reset-password";
import {
  ModifyUser,
  ModifyUserInput,
  ModifyUserOutput,
} from "../routes/user/modify";
import { validateJwt } from "./jwt";

export type Status = number;

export type MiddlewareResp = false | string;

export const ServerRoutes: Routes = {
  groups: [
    {
      prefix: "/auth",
      middleware: null,
      routes: {
        login: {
          method: "POST",
          endpoint: Login,
          input: LoginInput,
          output: LoginOutput,
        },
        register: {
          method: "POST",
          endpoint: Register,
          input: RegisterInput,
          output: RegisterOutput,
        },
        verify_email: {
          method: "POST",
          endpoint: VerifyEmail,
          input: VerifyEmailInput,
          output: VerifyEmailOutput,
        },
        reset_password_send: {
          method: "POST",
          endpoint: ResetPasswordSend,
          input: ResetPasswordSendInput,
          output: ResetPasswordSendOutput,
        },
        reset_password_complete: {
          method: "POST",
          endpoint: ResetPasswordComplete,
          input: ResetPasswordCompleteInput,
          output: ResetPasswordCompleteOutput,
        },
      },
    },
    {
      prefix: "/i",
      middleware: async (req: Request, resolve: any, reject: any) => {
        if (req.headers.get("Authorization")) {
          if (await validateJwt(req.headers.get("Authorization") ?? "")) {
            resolve("correct JWT");
          }
        }

        reject("You must be authenticated to access this route");
      },
      routes: {
        modify_user: {
          method: "POST",
          endpoint: ModifyUser,
          input: ModifyUserInput,
          output: ModifyUserOutput,
        },
      },
    },
  ],
};
