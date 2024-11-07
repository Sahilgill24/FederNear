import React from "react";

interface DropIndicatorProps {
  beforeId: string | null;
  column: "draft" | "published" | "trained";
}

const DropIndicator: React.FC<DropIndicatorProps> = ({ beforeId, column }) => {
  return (
    <div
      data-before={beforeId || "-1"}
      data-column={column}
      className="my-0.5 h-0.5 w-full bg-primary opacity-0"
    />
  );
};

export default DropIndicator;