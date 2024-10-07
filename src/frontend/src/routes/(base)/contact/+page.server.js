import { notificationWebhookClient } from "/src/lib/server/discord";

/** @type {import('./$types').Actions} */
export const actions = {
  default: async (event) => {
    const data = await event.request.formData();
    const firstName = data.get("first-name");
    const lastName = data.get("last-name");
    const company = data.get("company");
    const email = data.get("email");
    const phone = data.get("phone");
    const message = data.get("message");
    const agreed = data.get("switch-1-label");

    await notificationWebhookClient.send({
      username: "Contact Form",
      embeds: [
        {
          title: `Contact Form`,
          fields: [
            {
              name: "First Name",
              value: `${firstName}`,
              inline: true,
            },
            {
              name: "Last Name",
              value: `${lastName}`,
              inline: true,
            },
            {
              name: "Company",
              value: `${company}`,
              inline: true,
            },
            {
              name: "Phone",
              value: `${phone}`,
              inline: true,
            },
            {
              name: "Email",
              value: `${email}`,
              inline: false,
            },
            {
              name: "Message",
              value: `${message}`,
              inline: false,
            },
          ],
        },
      ],
    });
  },
};
