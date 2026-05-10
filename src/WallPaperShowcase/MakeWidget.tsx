import { convertFileSrc, invoke } from "@tauri-apps/api/core"

type MakeWidgetProps = {
  label: string,
  field: string,
  img_path: string,
  onPicked: () => void
}

export default function MakeWidget({ label, field, img_path, onPicked }: MakeWidgetProps) {
  const bg = img_path ? convertFileSrc(img_path) : undefined;

  console.log(img_path, bg)

  return (
    <div className="flex flex-col gap-2 items-center justify-center">
      <button onClick={async () => {
        await invoke("open_file_explorer", {
          field
        });
        onPicked();
      }}
        className="w-60 h-40 rounded-3xl bg-cover bg-center bg-no-repeat"
        style={{ backgroundImage: bg ? `url(${bg})` : undefined }}>

      </button>
      <h1 className="font-bold text-white">{label}</h1>
    </div >
  )
}
