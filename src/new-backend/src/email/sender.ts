import { Resend } from "resend";
import React from "react";
import ReactDOMServer from "react-dom/server";
import { TeamInviteJsx, type TeamInviteProps } from "./templates/team_invite";
import { WelcomeUserJsx, type WelcomeUserProps } from "./templates/welcome";
import {
  VerifyEmailJsx,
  type VerifyEmailProps,
} from "./templates/verify_email";
import { RESEND_EMAIL_KEY } from "../utils/env";
import {
  ResetPasswordJsx,
  type ResetPasswordProps,
} from "./templates/reset_password";

const resend = new Resend(RESEND_EMAIL_KEY);

/// Send the user a reset password message
export async function sendResetPasswordEmail(
  email: string,
  props: ResetPasswordProps,
) {
  await sendEmail(
    email,
    `Reset your password`,
    React.createElement(ResetPasswordJsx, props),
  );
}

/// Send a user welcome email
export async function sendUserWelcomeEmail(
  email: string,
  props: WelcomeUserProps,
) {
  await sendEmail(email, `Welcome`, React.createElement(WelcomeUserJsx, props));
}

/// Send a verify user email
export async function sendVerifyUserEmail(
  email: string,
  props: VerifyEmailProps,
) {
  await sendEmail(
    email,
    `Verify your email`,
    React.createElement(VerifyEmailJsx, props),
  );
}

/// Send a team invite email
export async function sendTeamInviteEmail(
  email: string,
  props: TeamInviteProps,
) {
  await sendEmail(
    email,
    `You've been invited to join ${props.teamName}`,
    React.createElement(TeamInviteJsx, props),
  );
}

/// Underlying email abstraction
async function sendEmail<T>(
  to: string,
  subject: string,
  element: React.FunctionComponentElement<T>,
) {
  resend.emails.send({
    from: "pw@based.ceo",
    to,
    subject,
    html: ReactDOMServer.renderToString(element),
  });
}

/*await sendTeamInviteEmail("coristine.e@northeastern.edu", {
  invitedByUsername: "aidanperry",
  teamName: "Ghostwire",
  inviteLink: "https://packetware.net/invite/123",
  invitedUsername: "Edward Coristine",
  invitedByEmail: "aidan@based.ceo",
});*/

/*await sendUserWelcomeEmail("pw@aidanperry.net", {
  name: "Aidan Perry",
});*/

/*await sendVerifyUserEmail("pw@aidanperry.net", {
  name: "Aidan Perry",
  verifyLink: "https://packetware.net/verify/123",
});*/
