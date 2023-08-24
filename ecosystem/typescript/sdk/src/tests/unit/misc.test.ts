// Copyright © Diem Foundation
// SPDX-License-Identifier: Apache-2.0

import { DiemClient } from "../../providers/diem_client";

test("test fixNodeUrl", () => {
  expect(new DiemClient("https://test.com").nodeUrl).toBe("https://test.com/v1");
  expect(new DiemClient("https://test.com/").nodeUrl).toBe("https://test.com/v1");
  expect(new DiemClient("https://test.com/v1").nodeUrl).toBe("https://test.com/v1");
  expect(new DiemClient("https://test.com/v1/").nodeUrl).toBe("https://test.com/v1");
  expect(new DiemClient("https://test.com", {}, true).nodeUrl).toBe("https://test.com");
});
