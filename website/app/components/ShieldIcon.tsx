type Props = { className?: string };

export function ShieldIcon({ className = "h-5 w-5" }: Props) {
  return (
    <svg
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      className={className}
    >
      <path
        d="M12 2.5 4 5v6.2c0 4.6 3.2 8.8 8 10.3 4.8-1.5 8-5.7 8-10.3V5l-8-2.5Z"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinejoin="round"
        fill="none"
      />
      <path
        d="m9 12 2.2 2.2L15 10.4"
        stroke="currentColor"
        strokeWidth="1.6"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}
