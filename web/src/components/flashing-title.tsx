import { useEffect, useState } from "react";

const FlashingTitle = () => {
  const [isOriginal, setIsOriginal] = useState(true);

  const originalTitle = "🧊🧊🧊 BOX... 🧊🧊🧊";
  const flashingTitle = "🌎🌎🌎 LAND 🌎🌎🌎";

  useEffect(() => {
    const interval = setInterval(() => {
      setIsOriginal((prev) => !prev);
    }, 1000); // Change every 1 second

    // Cleanup function to clear interval on unmount
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    document.title = isOriginal ? originalTitle : flashingTitle;
  }, [isOriginal]);

  return (<></>);
};

export default FlashingTitle;
