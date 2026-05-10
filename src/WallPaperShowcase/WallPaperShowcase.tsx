import { useEffect, useState } from "react"
import MakeWidget from "./MakeWidget"
import { invoke } from "@tauri-apps/api/core";

export default function WallPaperShowcase() {
  const [paths, setPaths] = useState({ morning: "", afternoon: "", evening: "" });

  const fetchPaths = () => {
    Promise.all([
      invoke<string>("get_wallpaper_morning"),
      invoke<string>("get_wallpaper_afternoon"),
      invoke<string>("get_wallpaper_evening"),
    ]).then(([morning, afternoon, evening]) => setPaths({ morning, afternoon, evening }));
  };

  useEffect(fetchPaths, []);

  return (
    <section className="gap-5 flex flex-col justify-center items-center">
      <div className="flex flex-row justify-center items-center gap-8">
        <MakeWidget field="morning" label="Morning" img_path={paths.morning} onPicked={fetchPaths} />
        <MakeWidget field="afternoon" label="Afternoon" img_path={paths.afternoon} onPicked={fetchPaths} />
      </div>
      < MakeWidget field="evening" label="Evening" img_path={paths.evening} onPicked={fetchPaths} />
    </section >
  )
}
