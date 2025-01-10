export function formatMessage(obj: any): string {
  if (Array.isArray(obj)) {
    return "[" + obj.map((item) => formatMessage(item)).join(",") + "]";
  } else if (typeof obj === "object") {
    return JSON.stringify(obj);
  }

  return new String(obj) as string;
}
