import WallPaperShowcase from "./WallPaperShowcase/WallPaperShowcase"

export default function App() {
  return (
    <main className="w-screen h-screen flex flex-col justify-center items-center bg-linear-to-b from-(--background-lighter) to-(--background)">

      <WallPaperShowcase></WallPaperShowcase>
    </main>
  )
}
