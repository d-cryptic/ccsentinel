import { AnnouncementBanner } from "./components/AnnouncementBanner";
import { Navbar } from "./components/Navbar";
import { Hero } from "./components/Hero";
import { Stats } from "./components/Stats";
import { Features } from "./components/Features";
import { HowItWorks } from "./components/HowItWorks";
import { TerminalDemo } from "./components/TerminalDemo";
import { Integrations } from "./components/Integrations";
import { OpenSource } from "./components/OpenSource";
import { FinalCTA } from "./components/FinalCTA";
import { Footer } from "./components/Footer";

export default function Home() {
  return (
    <>
      <AnnouncementBanner />
      <Navbar />
      <main>
        <Hero />
        <Stats />
        <Features />
        <HowItWorks />
        <TerminalDemo />
        <Integrations />
        <OpenSource />
        <FinalCTA />
      </main>
      <Footer />
    </>
  );
}
