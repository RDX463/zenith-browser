import '@testing-library/jest-dom'
import { afterEach, vi } from 'vitest'
import { cleanup } from '@testing-library/react'

// Cleanup after each test
afterEach(() => {
  cleanup()
})

// Mock IPC for testing
const mockWindow = global.window as any;
mockWindow.ipc = {
  postMessage: vi.fn(),
  onState: vi.fn(() => () => {}),
  onSuggestions: vi.fn(() => () => {}),
  send: vi.fn(),
};
