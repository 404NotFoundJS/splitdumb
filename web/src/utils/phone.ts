export function formatPhoneNumber(value: string): string {
  const digits = value.replace(/\D/g, "");
  if (digits.length <= 3) return digits;
  if (digits.length <= 7) return `${digits.slice(0, 3)}-${digits.slice(3)}`;
  return `${digits.slice(0, 3)}-${digits.slice(3, 7)}-${digits.slice(7, 11)}`;
}

export function getDigitCount(phone: string): number {
  return phone.replace(/\D/g, "").length;
}

export function isValidPhone(phone: string): boolean {
  return getDigitCount(phone) >= 10;
}
