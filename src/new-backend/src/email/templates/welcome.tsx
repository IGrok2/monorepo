import {
  Body,
  Button,
  Container,
  Column,
  Head,
  Heading,
  Hr,
  Html,
  Img,
  Link,
  Preview,
  Row,
  Section,
  Text,
  Tailwind,
} from "@react-email/components";
import * as React from "react";

const Image = "https://icons.fsn1.your-objectstorage.com/black_purple.png";

/// The props when a user signs up to the platform
export interface WelcomeUserProps {
  /// The username of the user being invited
  name: string;
}

export const WelcomeUserJsx = ({ name }: WelcomeUserProps) => {
  const previewText = `Welcome, ${name}`;

  return (
    <Html>
      <Head />
      <Preview>{previewText}</Preview>
      <Tailwind>
        <Body className="bg-white my-auto mx-auto font-sans px-2">
          <Container className="border border-solid border-[#eaeaea] rounded my-[40px] mx-auto p-[20px] max-w-[465px]">
            <Section className="mt-[32px]">
              <Img
                src={Image}
                width="90"
                height="37"
                alt="Packetware"
                className="my-0 mx-auto"
              />
            </Section>
            <Heading className="text-black text-[24px] font-normal text-center p-0 my-[30px] mx-0">
              Welcome to Packetware
            </Heading>
            <Text className="text-black text-[14px] leading-[24px]">
              Hello {name},
            </Text>
            <Text className="text-black text-[14px] leading-[24px]">
              <strong>
                Welcome to the platform you've been missing for a long time.
              </strong>{" "}
              For applications with velocity, Packetware is the most suitable
              platform to grow and scale. It has the core features you need to
              scale effortlessly. Includes out of the box:
              <br />
              {"- "}
              <strong>Automatic deploys</strong> with rollbacks
              <br />
              {"- "}
              <strong>Highest clock speed</strong> of any serverless platform
              <br />
              {"- "}
              <strong>Best network performance</strong> of any serverless
              platform (25Gbps+)
              <br />
              {"- "}
              <strong>Incredible cybersecurity features</strong> backed by ML
              and first principles
              <br />
              {"- "}
              <strong>High reliability</strong> backed by a 100% network uptime
              SLA
            </Text>
            <Hr className="border border-solid border-[#eaeaea] my-[26px] mx-0 w-full" />
            <Text className="text-[#666666] text-[12px] leading-[24px]">
              If you have any questions, please don't hesitate to reach out to
              us at{" "}
              <Link
                className="text-[#666666] underline"
                href="mailto:edward@based.ceo"
              >
                edward@based.ceo
              </Link>
              .
            </Text>
          </Container>
        </Body>
      </Tailwind>
    </Html>
  );
};
