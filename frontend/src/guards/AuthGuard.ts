import { useEffect } from "react";
import { useRouter } from "next/router";
import { useAppSelector } from "@/redux";

type Props = {
    children: React.ReactNode;
}

const AuthGuard = ({ children }: Props) => {
    const router = useRouter();
    const isUserLoggedIn = useAppSelector((state) => state?.user?.isLoggedIn);

    useEffect(() => {
        if (!isUserLoggedIn) {
            router.push("/account");
        }
    }, [isUserLoggedIn])

    return (
        <>{children}</>
    )
}

export default AuthGuard;