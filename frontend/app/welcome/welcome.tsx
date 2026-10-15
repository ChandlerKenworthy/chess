import ChessGame from "~/components/ChessGame";

export function Welcome() {
  return (
    <main className="flex flex-col items-center justify-center pt-16 pb-4 gap-6">
      <div className="flex flex-col items-center gap-2">
        <h1 className="text-3xl font-bold">Chandler's Chess Engine</h1>
        <p className="text-mist-500">Version: 0.0.1</p>
      </div>
      <ChessGame />
    </main>
  );
}
