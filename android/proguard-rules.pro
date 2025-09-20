# Fix R8 warnings for licensing services
     -keep public class com.google.vending.licensing.ILicensingService { void <init>(); }
     -keep public class com.android.vending.licensing.ILicensingService { void <init>(); }
     -keep public class com.google.android.vending.licensing.ILicensingService { void <init>(); }
     # Fix R8 warnings for annotations
     -keep class android.support.annotation.Keep { void <init>(); }
     -keep,allowshrinking public class androidx.webkit.WebViewClientCompat { void <init>(); }
     -keep class * implements androidx.lifecycle.LifecycleObserver { void <init>(); }
     -keep class * implements androidx.versionedparcelable.VersionedParcelable { void <init>(); }
     -keep public class androidx.versionedparcelable.ParcelImpl { void <init>(); }
     -keep,allowshrinking class * extends androidx.startup.Initializer { void <init>(); }
     -keep,allowrepackaging,allowobfuscation @interface androidx.annotation.Keep { void <init>(); }