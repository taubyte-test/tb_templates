import { Event } from "sdk/event";

export function ping(e: Event): u32 {
  let httpEvent = e.http();
  if (httpEvent == null) {
    return 1;
  }

  let resp = httpEvent.unwrap().write("PONG");
  if (resp.err) {
    return 1;
  }

  return 0;
}
